use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_74674755: FileFormat = FileFormat {
    id: 74_674_755,
    source_type: SourceType::Wikidata,
    name: "SPSS Table Look",
    extensions: &["tlo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
