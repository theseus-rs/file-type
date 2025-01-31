use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111009267: FileFormat = FileFormat {
    id: 111_009_267,
    puid: "wikidata/111009267",
    name: "PrintMaster Newsletter File format",
    extensions: &["nws"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
