use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47018772: FileFormat = FileFormat {
    id: 47_018_772,
    source_type: SourceType::Wikidata,
    name: "PageMaker Document file format, version 6.5",
    extensions: &["p65"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
