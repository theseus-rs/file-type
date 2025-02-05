use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66689208: FileFormat = FileFormat {
    id: 66_689_208,
    source_type: SourceType::Wikidata,
    name: "Access Database (Pocket Access for Windows CE)",
    extensions: &["cdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
