use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129167131: FileFormat = FileFormat {
    id: 129_167_131,
    source_type: SourceType::Wikidata,
    name: "Evoque file format",
    extensions: &["evoque"],
    media_types: &["application/x-evoque"],
    signatures: &[],
    related_formats: &[],
};
