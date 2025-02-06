use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857112: FileFormat = FileFormat {
    id: 105_857_112,
    source_type: SourceType::Wikidata,
    name: "XML Grammar",
    extensions: &["grxml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
