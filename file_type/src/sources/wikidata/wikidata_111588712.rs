use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111588712: FileFormat = FileFormat {
    id: 111_588_712,
    source_type: SourceType::Wikidata,
    name: "Roxio Label Creator Project File, versions 8-11",
    extensions: &["jwl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
