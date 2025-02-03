use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111272528: FileFormat = FileFormat {
    id: 111_272_528,
    source_type: SourceType::Wikidata,
    name: "Everest embedded bank file",
    extensions: &["emb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
