use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_85513175: FileFormat = FileFormat {
    id: 85_513_175,
    source_type: SourceType::Wikidata,
    name: "Cindex Document, version 2",
    extensions: &["cdx", "tpl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
