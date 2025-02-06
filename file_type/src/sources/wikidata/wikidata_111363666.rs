use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111363666: FileFormat = FileFormat {
    id: 111_363_666,
    source_type: SourceType::Wikidata,
    name: "Wusikstation file-pack",
    extensions: &["wusikpack"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
