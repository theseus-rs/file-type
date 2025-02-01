use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61739757: FileFormat = FileFormat {
    id: 61_739_757,
    puid: "wikidata/61739757",
    name: "Adobe FrameMaker Document, version 5",
    extensions: &["fm"],
    media_types: &["application/vnd.framemaker"],
    internal_signatures: &[],
    related_formats: &[],
};
