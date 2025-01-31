use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129425911: FileFormat = FileFormat {
    id: 129_425_911,
    puid: "wikidata/129425911",
    name: "Gosu template file",
    extensions: &["gst"],
    media_types: &["text/x-gosu-template"],
    internal_signatures: &[],
    related_formats: &[],
};
