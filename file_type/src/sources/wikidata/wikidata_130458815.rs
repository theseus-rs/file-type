use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130458815: FileFormat = FileFormat {
    id: 130_458_815,
    source_type: SourceType::Wikidata,
    name: "ParaSail source code",
    extensions: &["psi"],
    media_types: &["text/x-parasail"],
    internal_signatures: &[],
    related_formats: &[],
};
