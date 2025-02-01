use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128772411: FileFormat = FileFormat {
    id: 128_772_411,
    puid: "wikidata/128772411",
    name: "ClojureScript file format",
    extensions: &["cljs", "cljs"],
    media_types: &["application/x-clojurescript", "text/x-clojurescript"],
    internal_signatures: &[],
    related_formats: &[],
};
