use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129167288: FileFormat = FileFormat {
    id: 129_167_288,
    source_type: SourceType::Wikidata,
    name: "execline file format",
    extensions: &["exec"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
