use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51839234: FileFormat = FileFormat {
    id: 51_839_234,
    source_type: SourceType::Wikidata,
    name: "Inset Systems Bitmap",
    extensions: &["pix"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
