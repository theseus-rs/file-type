use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34285550: FileFormat = FileFormat {
    id: 34_285_550,
    source_type: SourceType::Wikidata,
    name: "Perl header",
    extensions: &["ph"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
