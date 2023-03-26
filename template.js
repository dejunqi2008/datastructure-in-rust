const fs = require('fs');
const path = require('path');
const ps = require('child_process');
const readline = require('readline');

const CURRENT_WORKSPACE = path.join(__dirname, './');
const LC_MOD_FILE_ROOT = `${CURRENT_WORKSPACE}/src/leetcode/mod.rs`;
const PROJECT_MAIN = `${CURRENT_WORKSPACE}/src/main.rs`; 


const run = () => {
    const modname = process.argv.slice(2)[0];

    if (!modname || fs.existsSync(`${CURRENT_WORKSPACE}/src/leetcode/${modname}`)) {
        console.log('\ndirectory already exist.')
        return;
    }

    console.log(CURRENT_WORKSPACE, modname);
    ps.execSync(`mkdir ${CURRENT_WORKSPACE}/src/leetcode/${modname}`)

    const lc_mod_path = `${CURRENT_WORKSPACE}/src/leetcode/${modname}/mod.rs`;
    const lc_code_path = `${CURRENT_WORKSPACE}/src/leetcode/${modname}/solution.rs`;
    ps.execSync(`touch ${lc_mod_path} ${lc_code_path}`);
    fs.writeFileSync(lc_mod_path, 'pub mod solution;');
    fs.writeFileSync(lc_code_path, 'pub struct Solution {}\n\nimpl Solution {\n\n\tpub fn test() {}\n}\n');
    fs.appendFileSync(LC_MOD_FILE_ROOT, `\npub mod ${modname};`)
}


run();

/*
use crate::leetcode::utils::list_node::ListNode;

pub struct Solution {
    reverse_list: fn(Option<Box<ListNode>>) -> Option<Box<ListNode>>
}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut prev: Option<Box<ListNode>> = None;
        let mut cur: &Option<Box<ListNode>> = &head;

        while cur.is_some() {
            let mut node: ListNode = ListNode::new(cur.as_ref()?.val);
            node.next = prev;
            prev = Some(Box::new(node));
            cur = &(
                    cur    // &Option<Box<ListNode>
                        .as_ref()? // as_ref() turns &Option<Box<ListNode> to Option<&Box<ListNode>>, then '?' takes take values inside the Option and unwrap it to &Box<ListNode>> 
                        .next
                    );
        }
        return prev;
    }
}

pub fn test() {
    
    let arr = [5, 4, 3, 2, 1];
    let mut head: Option<Box<ListNode>> = None;

    for i in 0..arr.len() {
        let node: Box<ListNode> = Box::new(ListNode {
            val: arr[i],
            next: head.take()
        });
        head = Some(node);
    }

    // println!("{:?}", head.as_ref().unwrap().val);

    let newhead = Solution::reverse_list(head);
}
*/