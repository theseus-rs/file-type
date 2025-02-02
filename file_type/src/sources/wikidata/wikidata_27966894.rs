use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27966894: FileFormat = FileFormat {
    id: 27_966_894,
    source_type: SourceType::Wikidata,
    name: "GSF",
    extensions: &["gsf", "gsflib", "minigsf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
