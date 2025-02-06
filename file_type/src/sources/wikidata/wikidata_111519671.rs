use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111519671: FileFormat = FileFormat {
    id: 111_519_671,
    source_type: SourceType::Wikidata,
    name: "PageMaker template file, version 5",
    extensions: &["pt5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
