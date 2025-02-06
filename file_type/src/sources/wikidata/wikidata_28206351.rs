use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206351: FileFormat = FileFormat {
    id: 28_206_351,
    source_type: SourceType::Wikidata,
    name: "Inset PIX",
    extensions: &["pix"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
