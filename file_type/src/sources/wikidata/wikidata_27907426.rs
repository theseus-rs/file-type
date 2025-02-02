use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27907426: FileFormat = FileFormat {
    id: 27_907_426,
    source_type: SourceType::Wikidata,
    name: "Amiga Disk File, compressed",
    extensions: &["adz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
