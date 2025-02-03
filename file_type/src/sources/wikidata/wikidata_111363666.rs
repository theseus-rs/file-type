use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111363666: FileFormat = FileFormat {
    id: 111_363_666,
    source_type: SourceType::Wikidata,
    name: "Wusikstation file-pack",
    extensions: &["wusikpack"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
