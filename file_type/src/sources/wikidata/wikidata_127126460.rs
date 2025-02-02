use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127126460: FileFormat = FileFormat {
    id: 127_126_460,
    source_type: SourceType::Wikidata,
    name: "Harwell-Boeing file",
    extensions: &["hb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
