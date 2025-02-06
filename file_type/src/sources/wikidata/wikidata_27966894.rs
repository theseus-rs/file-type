use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27966894: FileFormat = FileFormat {
    id: 27_966_894,
    source_type: SourceType::Wikidata,
    name: "GSF",
    extensions: &["gsf", "gsflib", "minigsf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
