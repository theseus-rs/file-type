use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_57978165: FileFormat = FileFormat {
    id: 57_978_165,
    puid: "wikidata/57978165",
    name: "ASP WebService Directive File",
    extensions: &["asmx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
