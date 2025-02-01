use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111009835: FileFormat = FileFormat {
    id: 111_009_835,
    puid: "wikidata/111009835",
    name: "PrintMaster Post Card File format",
    extensions: &["pcr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
