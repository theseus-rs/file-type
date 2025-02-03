use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_97062804: FileFormat = FileFormat {
    id: 97_062_804,
    source_type: SourceType::Wikidata,
    name: "X-Motif UIL table",
    extensions: &["uil"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
