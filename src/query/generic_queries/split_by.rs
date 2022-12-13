use crate::query::generic_query::{GenericQuery, VarOrValue};
use crate::runtime::frame::Frame;
use crate::runtime::value::Value as MyValue;

pub struct SplitBy;

impl SplitBy {
    pub fn query_var0(&self, frame: &Frame, args: &[VarOrValue]) -> Option<Vec<Frame>> {
        let result = String::from(args[0].as_value()?.as_loose_string()?)
            + &args[1].as_value()?.as_loose_string()?
            + &args[2].as_value()?.as_loose_string()?
            == args[3].as_value()?.as_loose_string()?;
        if result {
            Some(vec![frame.clone()])
        } else {
            None
        }
    }

    pub fn query_var1(&self, frame: &Frame, args: &[VarOrValue]) -> Option<Vec<Frame>> {
        let mut var_index = 0;
        for (index, item) in args.iter().enumerate() {
            if item.is_var() {
                var_index = index;
                break;
            }
        }
        let var_name = args[var_index].get_var_name()?;

        let mut new_frame = frame.clone();

        if var_index == 0 {
            let trailing = String::from(args[1].as_value()?.as_loose_string()?)
                + &args[2].as_value()?.as_loose_string()?;
            let all = &args[3].as_value()?.as_loose_string()?;
            if !all.ends_with(&trailing) {
                return None;
            }

            let remain = &all[..all.len() - trailing.len()];
            new_frame.add(var_name, MyValue::from_string(remain))
        } else if var_index == 1 {
            let leading = &args[0].as_value()?.as_loose_string()?;
            let trailing = &args[2].as_value()?.as_loose_string()?;
            let all = &args[3].as_value()?.as_loose_string()?;

            if !all.starts_with(leading) {
                return None;
            }
            if !all.ends_with(trailing) {
                return None;
            }
            let remain = &all[leading.len()..all.len() - trailing.len()];
            new_frame.add(var_name, MyValue::from_string(remain));
        } else if var_index == 2 {
            let leading = String::from(args[0].as_value()?.as_loose_string()?.as_str())
                + &args[1].as_value()?.as_loose_string()?;
            let all = &args[3].as_value()?.as_loose_string()?;
            if !all.starts_with(&leading) {
                return None;
            }
            let remain = &all[leading.len()..];
            new_frame.add(var_name, MyValue::from_string(remain));
        } else if var_index == 3 {
            let all = String::from(args[0].as_value()?.as_loose_string()?.as_str())
                + &args[1].as_value()?.as_loose_string()?
                + &args[2].as_value()?.as_loose_string()?;
            new_frame.add(var_name, MyValue::from_string(&all));
        }

        Some(vec![new_frame])
    }

    pub fn query_var2(&self, frame: &Frame, args: &[VarOrValue]) -> Option<Vec<Frame>> {
        let mut var_index1 = 0;
        let mut var_index2 = 0;
        for (index, item) in args.iter().enumerate() {
            if item.is_var() {
                var_index1 = index;
                for (i2, item2) in args.iter().enumerate().skip(index + 1) {
                    if item2.is_var() {
                        var_index2 = i2;
                        break;
                    }
                }
                break;
            }
        }

        let var_name1 = args[var_index1].get_var_name()?;
        let var_name2 = args[var_index2].get_var_name()?;
        let mut result = Vec::new();

        let get_str = |index: usize| -> Option<String> {
            Some(args[index].as_value()?.as_loose_string()?)
        };

        if var_index2 != 3 {
            let all = &args[3].as_value()?.as_loose_string()?;
            if var_index1 == 0 && var_index2 == 1 {
                let end = &args[2].as_value()?.as_loose_string()?;
                if !all.ends_with(end) {
                    return None;
                }
                let remain = &all[..all.len() - end.len()];

                for split_index in 0..=remain.len() {
                    let left = &remain[0..split_index];
                    let right = &remain[split_index..];
                    let mut new_frame = frame.clone();
                    new_frame.add(var_name1, MyValue::from_string(left));
                    new_frame.add(var_name2, MyValue::from_string(right));
                    result.push(new_frame);
                }
            } else if var_index1 == 0 && var_index2 == 2 {
                let middle = &get_str(1)?;
                for (i, _) in all.match_indices(middle) {
                    let left = &all[0..i];
                    let right = &all[i + middle.len()..];
                    let mut new_frame = frame.clone();
                    new_frame.add(var_name1, MyValue::from_string(left));
                    new_frame.add(var_name2, MyValue::from_string(right));
                    result.push(new_frame);
                }
            } else if var_index1 == 1 && var_index2 == 2 {
                let start = &get_str(0)?;
                if !all.starts_with(start) {
                    println!("{}, {}", all, start);
                    return None;
                }
                let remain = &all[start.len()..];
                for split_index in 0..=remain.len() {
                    let left = &remain[0..split_index];
                    let right = &remain[split_index..];
                    let mut new_frame = frame.clone();
                    new_frame.add(var_name1, MyValue::from_string(left));
                    new_frame.add(var_name2, MyValue::from_string(right));
                    result.push(new_frame);
                }
            }
        } else {
            panic!("in SplitBy, the 4th argument cannot be a var, because there will be infinite possibilities");
        }

        Some(result)
    }

    pub fn query_var3(&self, frame: &Frame, args: &[VarOrValue]) -> Option<Vec<Frame>> {
        if args[3].is_var() {
            panic!("in SplitBy, the 4th argument cannot be a var, because there will be infinite possibilities");
        }

        let vname1 = args[0].get_var_name()?;
        let vname2 = args[1].get_var_name()?;
        let vname3 = args[2].get_var_name()?;

        let mut result = Vec::new();
        let all = &args[3].as_value()?.as_loose_string()?;
        for i1 in 0..=all.len() {
            for i2 in i1..=all.len() {
                let left = &all[0..i1];
                let mid = &all[i1..i2];
                let right = &all[i2..];
                let mut new_frame = frame.clone();
                new_frame.add(vname1, MyValue::from_string(left));
                new_frame.add(vname2, MyValue::from_string(mid));
                new_frame.add(vname3, MyValue::from_string(right));
                result.push(new_frame);
            }
        }

        Some(result)
    }
}

impl GenericQuery for SplitBy {
    fn query(&self, input: &[Frame], args: &[VarOrValue]) -> Option<Vec<Frame>> {
        if args.len() != 4 {
            return None;
        }

        let mut result = Vec::new();
        for frame in input.iter() {
            let mut var_count = 0;
            let new_args = args.iter().map(|x| x.match_in_frame(frame)).collect::<Vec<_>>();
            for item in new_args.iter() {
                if item.is_var() {
                    var_count += 1;
                }
            }

            if var_count == 0 {
                result.append(&mut self.query_var0(frame, &new_args).unwrap_or(vec![]));
            } else if var_count == 1 {
                result.append(&mut self.query_var1(frame, &new_args).unwrap_or(vec![]));
            } else if var_count == 2 {
                result.append(&mut self.query_var2(frame, &new_args).unwrap_or(vec![]));
            } else if var_count == 3 {
                result.append(&mut self.query_var3(frame, &new_args).unwrap_or(vec![]));
            } else {
                panic!("in SplitBy, the 4th argument cannot be a var, because there will be infinite possibilities");
            }
        }
        // println!("{:?}", result);

        Some(result)
    }
}
