use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_57978165: FileFormat = FileFormat {
    id: 57_978_165,
    source_type: SourceType::Wikidata,
    name: "ASP WebService Directive File",
    extensions: &["asmx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
