use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131861742: FileFormat = FileFormat {
    id: 131_861_742,
    source_type: SourceType::Wikidata,
    name: "GE Signa ximg file",
    extensions: &["ximg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
