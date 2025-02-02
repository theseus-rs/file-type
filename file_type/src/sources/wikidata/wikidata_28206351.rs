use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206351: FileFormat = FileFormat {
    id: 28_206_351,
    source_type: SourceType::Wikidata,
    name: "Inset PIX",
    extensions: &["pix"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
