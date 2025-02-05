use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130458815: FileFormat = FileFormat {
    id: 130_458_815,
    source_type: SourceType::Wikidata,
    name: "ParaSail source code",
    extensions: &["psi"],
    media_types: &["text/x-parasail"],
    signatures: &[],
    related_formats: &[],
};
