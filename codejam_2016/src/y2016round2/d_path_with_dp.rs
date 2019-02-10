debug!("find a subset for merged node #{}", idx);

            //dp[ node_idx ][sum + N] = cheapest that sums to x using nodes up to node_idx
            let NON_INIT = 5000i16;
            let mut dp = vec![vec![NON_INIT; 2 * N + 2]; 1];
            for dp_idx in 0..merged_node_list.len() {
                let dp_node = merged_node_list[dp_idx].borrow();

                dp.push(dp[dp_idx].clone());

                let merged_node_index = dp_idx;
                //Because we have an initial dp vector with max sizes
                let dp_idx = dp_idx + 1;

                assert_eq!(dp_idx + 1, dp.len());

                if dp_node.diff() == 0 {
                    continue;
                }

                if used_node.get(merged_node_index) {
                    continue;
                }

                if merged_node_index == idx {
                    continue;
                }

                /*debug!("Looping through {} to= {}",
                max(-Ni16, -Ni16 + dp_node.diff()), min(Ni16, Ni16 + dp_node.diff())
                );*/

                for val in max(-Ni16, -Ni16 + dp_node.diff())..=min(Ni16, Ni16 + dp_node.diff()) {
                    dp[dp_idx][(val + Ni16) as usize] = min(
                        dp[dp_idx - 1][(val + Ni16) as usize],
                        dp[dp_idx - 1][(val - dp_node.diff() + Ni16) as usize]
                            + dp_node.num_workers(),
                    );
                }

                dp[dp_idx][(dp_node.diff() + Ni16) as usize] = min(
                    dp[dp_idx][(dp_node.diff() + Ni16) as usize],
                    dp_node.num_workers(),
                );
            }

            let mut chosen_subset = Vec::new();
            let mut last_element_idx = merged_node_list.len();

            let mut target_diff = {
                let node = merged_node_list[idx].borrow();
                -node.diff()
            };

            let mut lowest_worker_count = dp[last_element_idx][(Ni16 + target_diff) as usize];

            assert_ne!(NON_INIT, lowest_worker_count);

            'dp_path_loop: loop {
                //Find first element with the optimal size
                for dp_idx in 1..=merged_node_list.len() {
                    let value = dp[dp_idx][(Ni16 + target_diff) as usize];
                    /*debug!("For up to merged node {} for sum of {} smallest size is {}. ",
                    dp_idx-1, target_diff, value);*/

                    if value == lowest_worker_count {
                        chosen_subset.push(dp_idx - 1);
                        debug!(
                            "For up to merged node {} for sum of {} found optimal size {}. ",
                            dp_idx - 1,
                            target_diff,
                            lowest_worker_count
                        );

                        let node = merged_node_list[dp_idx - 1].borrow();
                        if node.diff() == target_diff {
                            break 'dp_path_loop;
                        }

                        lowest_worker_count = lowest_worker_count - node.num_workers();
                        assert!(lowest_worker_count >= 0);
                        target_diff = target_diff - node.diff();

                        assert!(dp_idx <= last_element_idx);
                        last_element_idx = dp_idx;

                        break;
                    }
                }
            }

            debug!("Merging all the chosen nodes");
            for chosen in chosen_subset {
                used_node.set(chosen, true);
                assert_ne!(chosen, idx);
                assert!(!Rc::ptr_eq(
                    &merged_node_list[idx],
                    &merged_node_list[chosen]
                ));

                merge_nodes(&merged_node_list[idx], &merged_node_list[chosen]);

                debug!("Chosen node {} to node {}", chosen, idx);

                merged_node_list[chosen] = merged_node_list[idx].clone();
            }

            let node = merged_node_list[idx].borrow();