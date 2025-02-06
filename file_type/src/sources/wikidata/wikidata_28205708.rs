use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205708: FileFormat = FileFormat {
    id: 28_205_708,
    source_type: SourceType::Wikidata,
    name: "Applixware Bitmap",
    extensions: &["im"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
