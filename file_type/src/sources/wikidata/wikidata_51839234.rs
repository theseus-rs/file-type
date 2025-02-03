use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51839234: FileFormat = FileFormat {
    id: 51_839_234,
    source_type: SourceType::Wikidata,
    name: "Inset Systems Bitmap",
    extensions: &["pix"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
