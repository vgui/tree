#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]


use std::ptr;
use std::vec::Vec;
use std::sync::atomic::{AtomicUsize, Ordering};
use core::any::Any;
use std::marker::PhantomData;


pub trait Tree
{   
    fn parent(&self) -> Option<TreeNode>;    
    fn remove(&mut self, childindex : usize);
    fn insert(&mut self,  childindex : usize, data : Box<dyn Any>);
    fn child(&mut self, index : usize) -> &mut TreeNode;
    fn childindex(&self) -> usize;
    fn children_count(&self) -> usize;
    fn data(&mut self) -> Option<&Box<dyn Any>>;
}


#[derive(Debug)]
pub struct TreeNode<'a>
{
    parent : Option<&'a Self>,
    children : Vec<Box<Self>>,
    childindex : usize,
    data : Box<dyn Any>,
}


impl<'a> TreeNode<'a>
{
    fn newroot(data : Box<dyn Any>) -> Box<Self>
    {
        let root = Box::new(Self
        {
            parent : None,
            children : Vec::new(),
            childindex : usize::MAX,
            data,
        });

        root
    }

    fn new(parent : &'a mut Self, childindex : usize, data : Box<dyn Any>) -> &Box<Self>
    {
        let node = Box::new(Self
        {
            parent : Some(parent),
            children : Vec::new(),
            childindex : usize::MAX,
            data,
        });

        let mut childindex = childindex;

        if childindex == usize::MAX
        {
            childindex = parent.children.len();    
        }

            //noderef.parent = parent;
        parent.children.insert(childindex, node);
        parent.children[childindex].update_indexes(childindex);                    

        &parent.children[childindex]
    }

    fn update_indexes(&mut self, start_index : usize)
    {
        let mut index : usize = start_index;

        while index < self.children.len()
        {
            self.children[index].childindex = index;
            index += 1
        }
    }

    pub fn remove(&mut self, childindex : usize) -> Box<Self>
    {
        if childindex >= self.children.len()
        {
            panic!("Too big child index for removing.");
        }
      
        let mut child = self.children.remove(childindex);
        child.parent = None;
        child.childindex = usize::MAX;
        self.update_indexes(childindex);

        child
    }

    pub fn insert(&mut self,  childindex : usize, child : &mut Self)
    {
        //Check child index.
        let mut childindex = childindex;

        if childindex == usize::MAX
        {
            childindex = self.children.len();   
        }

        if childindex > self.children.len()
        {
            panic!("Too big child index for inserting.");
        }

        if let Some(parent) = child.parent()
        {
            parent.remove(child.childindex);
        }

        self.children.insert(childindex, child);        
        self.update_indexes(childindex+1);
        child.parent = &self;
        child.childindex = childindex;        
    }

    fn parent(&self) -> Option<&Box<Self>>
    {
        self.parent
    }

    fn child(&mut self, index : usize) -> &Box<Self>
    {
        &self.children[index]
    }

    fn childindex(&self) -> usize
    {
        self.childindex
    }

    fn data(&self) -> &Box<dyn Any>
    {
        &self.data
    }    
}


#[cfg(test)]
mod tests 
{
    use super::*;
    use crate::tree;

    #[derive(PartialEq)]
    struct WidgetObj
    {
        id : String,
    }

    impl WidgetObj
    {
        pub fn new(id : &str) -> Box<WidgetObj>
        {
            Box::new(WidgetObj { id : String::from(id), })
        }
    }

    pub trait Widget
    {   
        fn paint(&mut self);
        fn size(&mut self);
    }

    impl Widget for WidgetObj
    {
        fn paint(&mut self){}
        fn size(&mut self) {}
    }

    #[test]
    fn test_treenode_new()
    {
        let root : Box<TreeNode> = TreeNode::new(None, usize::MAX, WidgetObj::new("root"));     
        let w1 : Box<TreeNode> = TreeNode::new(Some(&root), 0, WidgetObj::new("w1"));
        let w2 : Box<TreeNode> = TreeNode::new(Some(&root), 0, WidgetObj::new("w2"));
        let w3 : Box<TreeNode> = TreeNode::new(Some(&root), 0, WidgetObj::new("w3"));
                

        assert!(root.parent().is_none());
        assert_eq!(root.childindex(), usize::MAX);
            
        assert!(ptr::eq(w1.parent().unwrap().as_ref() , root.as_ref()));
        // assert_eq!(w1.childindex(),  0);

        assert!(ptr::eq(w2.parent().unwrap().as_ref() , root.as_ref()));
        // assert_eq!(w2.childindex(),  0);

        assert!(ptr::eq(w3.parent().unwrap().as_ref() , root.as_ref()));
        // assert_eq!(w3.childindex(),  0);       

    }

}//tests
