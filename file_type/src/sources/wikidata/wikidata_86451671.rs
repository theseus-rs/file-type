use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_86451671: FileFormat = FileFormat {
    id: 86_451_671,
    puid: "wikidata/86451671",
    name: "RFFlow Chart 4",
    extensions: &["flo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
