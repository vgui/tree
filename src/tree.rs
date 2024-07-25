#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::vec::Vec;
use std::sync::atomic::{AtomicUsize, Ordering};
use core::any::Any;
use std::marker::PhantomData;

pub trait Tree<T>
{   
    fn parent(&self) -> Option<TreeNode<T>>;    
    fn remove(&mut self, childindex : usize);
    fn insert(&mut self,  childindex : usize, data : T);
    fn child(&mut self, index : usize) -> &mut TreeNode<T>;
    fn childindex(&self) -> usize;
    fn children_count(&self) -> usize;
    fn data(&mut self) -> Option<&T>;
}


//#[derive(PartialEq)]
pub struct TreeNode<'a, T>
{
    parent : Option<&'a Box<TreeNode<'a, T>>>,
    children : Vec<Box<TreeNode<'a, T>>>,
    childindex : usize,
    data : Box<dyn Any + PartialEq>,
    _marker : PhantomData<T>,

}

impl<'a, T> PartialEq for Box<TreeNode<'a, T>>
{
    fn eq(&self, other: &Box<TreeNode<'a, T>>) -> bool
    {
        &self.as_ref() == &other.as_ref()     
    }
}

impl<'a, T> TreeNode<'a, T>
{
    fn new(parent : Option<&'a Box<TreeNode<'a, T>>>, childindex : usize, data : Box<dyn Any>) -> Box<Self>
    {
        Box::new(Self
        {
            parent,
            children : Vec::new(),
            childindex,
            data,
            _marker : PhantomData,
        })
    }

    fn parent(&self) -> Option<&'a Box<TreeNode<'a, T>>>
    {
        self.parent
    }

    fn child(&mut self, index : usize) -> &Box<TreeNode<T>>
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
        fn paint(&mut self){}
        fn size(&mut self) {}
    }

    // #[test]
    // fn test_treenode_new()
    // {
    //     let root = TreeNode<WidgetObj
    // }

    #[test]
    fn test_treenode_new()
    {
        let root : Box<TreeNode<WidgetObj>> = TreeNode::new(None, usize::MAX, WidgetObj::new("root"));     
        let w1 : Box<TreeNode<WidgetObj>> = TreeNode::new(Some(&root), usize::MAX, WidgetObj::new("w1"));
        let w2 : Box<TreeNode<WidgetObj>> = TreeNode::new(Some(&root), usize::MAX, WidgetObj::new("w2"));
        let w3 : Box<TreeNode<WidgetObj>> = TreeNode::new(Some(&root), usize::MAX, WidgetObj::new("w3"));
        
        if let None = root.parent()
        {
            println!("OK");
        }

        if let Some(&ref root) = w1.parent()
        {
            println!("OK");
        }

        if let Some(&ref root) = w1.parent()
        {
            println!("OK");
        }

        assert!(&*w1.parent().unwrap() == &root);       

        assert!(root.parent() == None);
        // assert_eq!(root.childindex(), usize::MAX);
            
        // assert_eq!(w1.parent(), Some(root));
        // assert_eq!(w1.childindex(),  usize::MAX);

        // assert_eq!(w2.parent(), Some(root));
        // assert_eq!(w2.childindex(),  usize::MAX);

        // assert_eq!(w3.parent(), Some(root));
        // assert_eq!(w3.childindex(),  usize::MAX);       

    }

}//tests
