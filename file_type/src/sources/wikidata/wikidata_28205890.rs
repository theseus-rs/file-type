use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205890: FileFormat = FileFormat {
    id: 28_205_890,
    source_type: SourceType::Wikidata,
    name: "Multipage Z-Soft Paintbrush Bitmap Graphics",
    extensions: &["dcx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
