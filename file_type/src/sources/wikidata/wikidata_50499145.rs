use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50499145: FileFormat = FileFormat {
    id: 50_499_145,
    source_type: SourceType::Wikidata,
    name: "QuickDraw 3D Metafile (ASCII)",
    extensions: &["3dmf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
