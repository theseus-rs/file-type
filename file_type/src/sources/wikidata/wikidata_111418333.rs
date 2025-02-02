use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111418333: FileFormat = FileFormat {
    id: 111_418_333,
    source_type: SourceType::Wikidata,
    name: "Adobe Bridge URL File",
    extensions: &["adobebridge"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
