use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131703099: FileFormat = FileFormat {
    id: 131_703_099,
    source_type: SourceType::Wikidata,
    name: "VERA output file",
    extensions: &["h5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
