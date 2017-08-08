use std::cmp;
use std::iter;
use cairo::Context;

use utils::Lerp;
use super::segments::BezierCurve;
use super::single::{Graphic, Group};

pub struct GroupsRaw(Vec<usize>);

impl GroupsRaw {
    pub fn link<'a, 'b>(self, graphic: &'b MorphGraphic<'a>)
        -> GroupsLinked<'a, 'b>
    {
        let mid = graphic.beziers.len() / 2;
        let groups = self.0.split_at(mid);
        let groups = groups.0.iter().zip(groups.1);
        let beziers = graphic.beziers.split_at(mid);
        let beziers = beziers.0.iter().zip(beziers.1);

        let mut start = graphic.start.groups()
            .map(BezierGroup::new).collect::<Vec<_>>();
        let mut target = graphic.target.groups()
            .map(BezierGroup::new).collect::<Vec<_>>();

        for (group, bezier) in groups.zip(beziers) {
            let group = (*group.0, *group.1);

            let target_group = &graphic.target.group(group.1);
            let segment = MorphSegment::new(bezier, target_group);
            start[group.0].add(segment);
            let start_group = &graphic.start.group(group.0);
            let segment = MorphSegment::new(bezier, start_group);
            target[group.1].add(segment);
        }

        GroupsLinked {start, target}
    }
}

pub struct GroupsLinked<'a: 'b, 'b> {
    start: Vec<BezierGroup<'a, 'b>>,
    target: Vec<BezierGroup<'a, 'b>>
}

impl<'a, 'b> GroupsLinked<'a, 'b> {
    fn choose(&self, t: f64) -> &[BezierGroup<'a, 'b>] {
        if t < 0.5 {
            &self.start
        } else {
            &self.target
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct MorphGraphic<'a> {
    pub start: &'a Graphic,
    pub target: &'a Graphic,
    beziers: Vec<BezierCurve>
}

impl<'a> MorphGraphic<'a> {
    pub fn new<'b>(start: &'a Graphic, target: &'a Graphic)
        -> (MorphGraphic<'a>, GroupsRaw)
    {
        let start_count = start.count_beziers();
        let target_count = target.count_beziers();
        let count = cmp::max(start_count, target_count);

        let mut graphic = MorphGraphic {
            start: start,
            target: target,
            beziers: Vec::with_capacity(2 * count)
        };

        let mut groups = Vec::with_capacity(2 * count);
        graphic.append_beziers(start, start_count, count, &mut groups);
        graphic.append_beziers(target, target_count, count, &mut groups);

        (graphic, GroupsRaw(groups))
    }

    fn append_beziers(&mut self, graphic: &Graphic,
        graphic_count: usize, count: usize, groups: &mut Vec<usize>)
    {
        let mut segment_id = 0;

        for (group_id, group) in graphic.groups().enumerate() {
            for segment in group.segments() {
                let mut splits = segment.count_beziers();
                splits += count / graphic_count - 1;

                if segment_id < count % graphic_count {
                    splits += 1;
                }

                groups.extend(iter::repeat(group_id).take(splits));
                let splits = segment.to_beziers(splits);
                self.beziers.extend(splits);
                segment_id += 1;
            }
        }
    }

    pub fn draw<'b>(&'b self, ctx: &Context, groups: GroupsLinked<'a, 'b>,
        t: f64)
    {
        let color = self.start.color().lerp(self.target.color(), t);

        for group in groups.choose(t) {
            group.draw(ctx, t, color);
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct BezierGroup<'a: 'b, 'b> {
    segments: Vec<MorphSegment<'a, 'b>>,
    group: &'a Group
}

impl<'a, 'b> BezierGroup<'a, 'b> {
    fn new(group: &'a Group) -> BezierGroup<'a, 'b> {
        BezierGroup {segments: Vec::new(), group}
    }

    fn add(&mut self, segment: MorphSegment<'a, 'b>) {
        self.segments.push(segment);
    }

    fn draw(&self, ctx: &Context, t: f64, color: (f64, f64, f64)) {
        let u = if t < 0.5 {t} else {1.0 - t};

        ctx.set_source_rgb(color.0, color.1, color.2);
        let mut segments = Vec::new();

        for segment in &self.segments {
            let line_width = self.group.line_width().lerp(&segment.group.line_width(), u);
            let segment = segment.start.lerp(&segment.target, t);
            segment.draw(ctx, false);
            ctx.set_line_width(line_width);
            ctx.stroke();
            segments.push(segment);
        }

        let t = (1.0 - 2.0 * t).abs();

        ctx.set_source_rgba(color.0, color.1, color.2, t);
        let mut begin = true;

        for segment in segments {
            segment.draw(ctx, begin);
            begin = false;
        }

        if self.group.close() {
            ctx.close_path();
        }

        ctx.set_line_width(t * self.group.line_width());
        self.group.style().paint(ctx);
    }
}

#[derive(Clone, PartialEq, Debug)]
struct MorphSegment<'a, 'b> {
    start: &'b BezierCurve,
    target: &'b BezierCurve,
    group: &'a Group
}

impl<'a, 'b> MorphSegment<'a, 'b> {
    fn new((start, target): (&'b BezierCurve, &'b BezierCurve),
        group: &'a Group) -> MorphSegment<'a, 'b>
    {
        MorphSegment {start, target, group}
    }
}
