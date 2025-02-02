use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131861742: FileFormat = FileFormat {
    id: 131_861_742,
    source_type: SourceType::Wikidata,
    name: "GE Signa ximg file",
    extensions: &["ximg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
