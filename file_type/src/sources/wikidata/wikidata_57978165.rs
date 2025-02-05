use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_57978165: FileFormat = FileFormat {
    id: 57_978_165,
    source_type: SourceType::Wikidata,
    name: "ASP WebService Directive File",
    extensions: &["asmx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
