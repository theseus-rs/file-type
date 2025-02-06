use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_104889134: FileFormat = FileFormat {
    id: 104_889_134,
    source_type: SourceType::Wikidata,
    name: "Propellerhead Reason Project File",
    extensions: &["reason", "rns"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
