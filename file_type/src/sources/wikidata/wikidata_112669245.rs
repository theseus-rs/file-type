use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112669245: FileFormat = FileFormat {
    id: 112_669_245,
    source_type: SourceType::Wikidata,
    name: "Lightscape Layers",
    extensions: &["lay"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
