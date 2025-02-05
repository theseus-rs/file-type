use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111588438: FileFormat = FileFormat {
    id: 111_588_438,
    source_type: SourceType::Wikidata,
    name: "Roxio Label Creator Project File, versions 6-7",
    extensions: &["jwl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
