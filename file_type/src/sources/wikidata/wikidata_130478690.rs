use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130478690: FileFormat = FileFormat {
    id: 130_478_690,
    puid: "wikidata/130478690",
    name: "Pike source code file",
    extensions: &["pike"],
    media_types: &["text/x-pike"],
    internal_signatures: &[],
    related_formats: &[],
};
