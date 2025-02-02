use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29651167: FileFormat = FileFormat {
    id: 29_651_167,
    source_type: SourceType::Wikidata,
    name: "Personal Address Book",
    extensions: &["pab"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
