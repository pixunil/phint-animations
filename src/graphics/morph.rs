use std::cmp;
use std::rc::Rc;
use rand::{self, Rng};
use cairo::Context;

use utils::Lerp;
use super::segments::BezierCurve;
use super::single::{Graphic, Group};

#[derive(Clone, Debug)]
struct GroupRaw {
    group_id: usize,
    pos: usize
}

impl GroupRaw {
    fn new(group_id: usize, pos: usize) -> GroupRaw {
        GroupRaw {group_id, pos}
    }
}

#[derive(Clone)]
pub struct GroupsRaw(Vec<GroupRaw>);

impl GroupsRaw {
    pub fn link<'a>(&self, graphic: &'a MorphGraphic) -> GroupsLinked<'a> {
        let mid = graphic.beziers.len() / 2;
        let groups = self.0.split_at(mid);
        let groups = groups.0.iter().zip(groups.1);
        let beziers = graphic.beziers.split_at(mid);
        let beziers = beziers.0.iter().zip(beziers.1);

        let mut start = vec![Vec::new(); graphic.start.groups().len()];
        let mut target = vec![Vec::new(); graphic.target.groups().len()];

        for (group, bezier) in groups.zip(beziers) {
            let pos = (group.0.pos, group.1.pos);
            let group = (group.0.group_id, group.1.group_id);

            let target_group = &graphic.target.group(group.1);
            let segment = MorphSegment::new(bezier, target_group);
            start[group.0].push((segment, pos.0));
            let start_group = &graphic.start.group(group.0);
            let segment = MorphSegment::new(bezier, start_group);
            target[group.1].push((segment, pos.1));
        }

        let start = start.into_iter().zip(graphic.start.groups())
            .map(|(mut segments, group)| {
                segments.sort_by_key(|&(_, pos)| pos);

                let segments = segments.into_iter()
                    .map(|(segment, _)| segment).collect();

                BezierGroup::new(segments, group)
            }).collect();

        let target = target.into_iter().zip(graphic.target.groups())
            .map(|(mut segments, group)| {
                segments.sort_by_key(|&(_, pos)| pos);

                let segments = segments.into_iter()
                    .map(|(segment, _)| segment).collect();

                BezierGroup::new(segments, group)
            }).collect();

        GroupsLinked {start, target}
    }
}

pub struct GroupsLinked<'a> {
    start: Vec<BezierGroup<'a>>,
    target: Vec<BezierGroup<'a>>
}

impl<'a> GroupsLinked<'a> {
    fn choose(&self, t: f64) -> &[BezierGroup<'a>] {
        if t < 0.5 {
            &self.start
        } else {
            &self.target
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct MorphGraphic {
    pub start: Rc<Graphic>,
    pub target: Rc<Graphic>,
    beziers: Vec<BezierCurve>
}

impl MorphGraphic {
    pub fn new(start: Rc<Graphic>, target: Rc<Graphic>)
        -> (MorphGraphic, GroupsRaw)
    {
        let start_count = start.count_beziers();
        let target_count = target.count_beziers();
        let count = cmp::max(start_count, target_count);

        let mut graphic = MorphGraphic {
            start: start.clone(),
            target: target.clone(),
            beziers: Vec::with_capacity(2 * count)
        };

        let mut groups = GroupsRaw(Vec::with_capacity(2 * count));
        graphic.append_beziers(&start, start_count, count, &mut groups);
        graphic.append_beziers(&target, target_count, count, &mut groups);

        (graphic, groups)
    }

    fn append_beziers(&mut self, graphic: &Graphic,
        graphic_count: usize, count: usize, groups: &mut GroupsRaw)
    {
        let mut combined = Vec::new();
        let mut segment_id = 0;
        let mut bezier_id = 0;

        for (group_id, group) in graphic.groups().enumerate() {
            for segment in group.segments() {
                let mut splits = segment.count_beziers();
                splits += count / graphic_count - 1;

                if segment_id < count % graphic_count {
                    splits += 1;
                }

                let splits = segment.to_beziers(splits).into_iter()
                    .map(|bezier| {
                        bezier_id += 1;
                        (bezier, group_id, bezier_id)
                    });

                combined.extend(splits);
                segment_id += 1;
            }
        }

        let mut rng = rand::thread_rng();
        rng.shuffle(&mut combined);

        for (bezier, group_id, pos) in combined {
            self.beziers.push(bezier);
            groups.0.push(GroupRaw::new(group_id, pos));
        }
    }

    pub fn draw<'a>(&'a self, ctx: &Context, groups: GroupsLinked<'a>, t: f64) {
        let color = self.start.color().lerp(self.target.color(), t);

        for group in groups.choose(t) {
            group.draw(ctx, t, color);
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct BezierGroup<'a> {
    segments: Vec<MorphSegment<'a>>,
    group: &'a Group
}

impl<'a> BezierGroup<'a> {
    fn new(segments: Vec<MorphSegment<'a>>, group: &'a Group) -> BezierGroup<'a> {
        BezierGroup {segments, group}
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
struct MorphSegment<'a> {
    start: &'a BezierCurve,
    target: &'a BezierCurve,
    group: &'a Group
}

impl<'a> MorphSegment<'a> {
    fn new((start, target): (&'a BezierCurve, &'a BezierCurve),
        group: &'a Group) -> MorphSegment<'a>
    {
        MorphSegment {start, target, group}
    }
}
