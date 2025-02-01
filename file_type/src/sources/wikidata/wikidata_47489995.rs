use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47489995: FileFormat = FileFormat {
    id: 47_489_995,
    puid: "wikidata/47489995",
    name: "Adobe FrameMaker Document, version 6",
    extensions: &["fm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
