use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29651167: FileFormat = FileFormat {
    id: 29_651_167,
    source_type: SourceType::Wikidata,
    name: "Personal Address Book",
    extensions: &["pab"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
