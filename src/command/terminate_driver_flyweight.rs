/*
 * Copyright 2020 UT OVERSEAS INC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::command::correlated_message_flyweight::{CorrelatedMessageDefn, CorrelatedMessageFlyweight};
use crate::concurrent::atomic_buffer::AtomicBuffer;
use crate::utils::types::Index;

/**
 * Command message flyweight to ask the driver process to terminate
 *
 * @see ControlProtocolEvents
 * <pre>
 *   0                   1                   2                   3
 *   0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
 *  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 *  |                       Correlation ID                          |
 *  |                                                               |
 *  +---------------------------------------------------------------+
 *  |                        Token Length                           |
 *  +---------------------------------------------------------------+
 *  |                        Token Buffer                          ...
 * ...                                                              |
 *  +---------------------------------------------------------------+
 * </pre>
 */

#[repr(C, packed(4))]
#[derive(Copy, Clone)]
struct TerminateDriverDefn {
    correlated_message: CorrelatedMessageDefn,
    token_length: Index,
}

const TERMINATE_DRIVER_LENGTH: Index = std::mem::size_of::<TerminateDriverDefn>() as Index;

pub(crate) struct PublicationMessageFlyweight {
    correlated_message_flyweight: CorrelatedMessageFlyweight,
    m_struct: TerminateDriverDefn,
}

impl PublicationMessageFlyweight {
    pub fn new(buffer: AtomicBuffer, offset: Index) -> Self {
        let correlated_message_flyweight = CorrelatedMessageFlyweight::new(buffer, offset);
        let m_struct = correlated_message_flyweight.flyweight.get::<TerminateDriverDefn>(0);
        Self {
            correlated_message_flyweight,
            m_struct,
        }
    }

    #[inline]
    pub fn token_buffer(&self) {
        unimplemented!()
    }

    #[inline]
    pub fn token_length(&self) -> Index {
        self.m_struct.token_length
    }

    #[inline]
    pub fn set_token_buffer(&mut self) {
        unimplemented!()
    }

    #[inline]
    pub fn length(&self) -> Index {
        TERMINATE_DRIVER_LENGTH + self.m_struct.token_length
    }
}