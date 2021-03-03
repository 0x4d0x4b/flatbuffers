/*
 * Copyright 2021 Google Inc. All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::primitives::*;
use crate::Vector;

/// A trait defening a Tag type for union offsets
pub trait TaggedUnion {
    type Tag;
}

/// Combines a union's value offset with its tag (discriminant) value
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TaggedWIPOffset<T: TaggedUnion> {
    pub tag: T::Tag,
    pub value: WIPOffset<T>,
}

/// Combines a vector of unions' values offsets with a vector
/// of their associated tags (discriminant) values
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TaggedVectorWIPOffset<'a, T: TaggedUnion> {
    pub tags: WIPOffset<Vector<'a, T::Tag>>,
    pub values: WIPOffset<Vector<'a, ForwardsUOffset<T>>>,
}

/// A trait that is implemented by TaggedUnion to enable conversion
/// of a value's WIPOffset to its tagged equivalent
pub trait TagUnionValueOffset<T>: Sized + TaggedUnion {
    fn from_value_offset(o: WIPOffset<T>) -> TaggedWIPOffset<Self>;
}

/// Used to create a slice of the same type TaggedWIPOffsets
/// from a sequence of union variant offsets
#[macro_export]
macro_rules! union_value_offsets {
   ( $union_name:ty $(, $value_offset:ident)* ) => {
       [ $(<$union_name>::from_value_offset($value_offset), )* ]
    };
}

