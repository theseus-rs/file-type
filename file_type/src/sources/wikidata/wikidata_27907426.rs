use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27907426: FileFormat = FileFormat {
    id: 27_907_426,
    source_type: SourceType::Wikidata,
    name: "Amiga Disk File, compressed",
    extensions: &["adz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
