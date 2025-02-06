use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975874: FileFormat = FileFormat {
    id: 28_975_874,
    source_type: SourceType::Wikidata,
    name: "OOGL TLIST Group file",
    extensions: &["grp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
