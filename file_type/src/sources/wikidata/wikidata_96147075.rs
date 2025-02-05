use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_96147075: FileFormat = FileFormat {
    id: 96_147_075,
    source_type: SourceType::Wikidata,
    name: "Wolfram machine learning format",
    extensions: &["wmlf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
